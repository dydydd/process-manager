use std::collections::HashMap;
use crate::process_info::collect_processes;

struct TreeNode {
    pid: u32,
    name: String,
    depth: usize,
    is_last: bool,
    parent_last_flags: Vec<bool>,
}

pub fn show_tree(filter: &Option<String>) {
    let mut sys = sysinfo::System::new_all();
    let procs = collect_processes(&mut sys);

    // Build parent -> children map and pid -> name map
    let mut children_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut proc_map: HashMap<u32, String> = HashMap::new();

    for p in &procs {
        proc_map.insert(p.pid, p.name.clone());
        // Skip self-referential parent (e.g., PID 0 on Windows has parent_pid = 0)
        if p.pid != p.parent_pid {
            children_map.entry(p.parent_pid).or_default().push(p.pid);
        }
    }

    // Sort children for consistent output
    for children in children_map.values_mut() {
        children.sort();
    }

    // Find root processes: those whose parent is not in the process list,
    // or whose parent is 0 (the Idle process is a special root)
    let mut roots: Vec<u32> = procs.iter()
        .filter(|p| {
            // PID 0 is always a root
            if p.pid == 0 {
                return true;
            }
            // Processes with parent 0 that aren't 0 itself are children of Idle
            // so they're not roots
            if p.parent_pid == 0 {
                return false;
            }
            // Otherwise, root if parent not in process map
            !proc_map.contains_key(&p.parent_pid)
        })
        .map(|p| p.pid)
        .collect();
    roots.sort();

    println!("\nProcess Tree\n");

    let filter_lower = filter.as_ref().map(|f| f.to_lowercase());

    // Track visited PIDs to prevent infinite loops from cycle bugs
    let mut visited = std::collections::HashSet::new();

    // Iterative DFS using a stack
    let mut stack: Vec<TreeNode> = roots.into_iter().rev().map(|pid| {
        TreeNode {
            pid,
            name: proc_map.get(&pid).cloned().unwrap_or_else(|| "<unknown>".into()),
            depth: 0,
            is_last: true,
            parent_last_flags: Vec::new(),
        }
    }).collect();

    while let Some(node) = stack.pop() {
        // Prevent cycles
        if !visited.insert(node.pid) {
            continue;
        }

        // If filter is set, check if this process or any descendant matches
        if let Some(keyword) = filter_lower.as_deref() {
            if !node.name.to_lowercase().contains(keyword)
                && !has_matching_descendant(node.pid, &children_map, &proc_map, keyword)
            {
                continue;
            }
        }

        // Build prefix
        let mut prefix = String::new();
        for (_i, &is_parent_last) in node.parent_last_flags.iter().enumerate().skip(1) {
            if is_parent_last {
                prefix.push_str("    ");
            } else {
                prefix.push_str("│   ");
            }
        }

        let connector = if node.is_last { "└── " } else { "├── " };

        if node.depth == 0 {
            println!("{} (PID: {})", node.name, node.pid);
        } else {
            println!("{}{}{} (PID: {})", prefix, connector, node.name, node.pid);
        }

        // Push children in reverse order so they pop in correct order
        if let Some(children) = children_map.get(&node.pid) {
            for (i, child) in children.iter().enumerate().rev() {
                let is_last_child = i == children.len() - 1;
                let mut parent_flags = node.parent_last_flags.clone();
                parent_flags.push(node.is_last);

                stack.push(TreeNode {
                    pid: *child,
                    name: proc_map.get(child).cloned().unwrap_or_else(|| "<unknown>".into()),
                    depth: node.depth + 1,
                    is_last: is_last_child,
                    parent_last_flags: parent_flags,
                });
            }
        }
    }
}

fn has_matching_descendant(
    pid: u32,
    children_map: &HashMap<u32, Vec<u32>>,
    proc_map: &HashMap<u32, String>,
    keyword: &str,
) -> bool {
    let mut stack = vec![pid];
    let mut visited = std::collections::HashSet::new();
    while let Some(current) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }
        if let Some(children) = children_map.get(&current) {
            for child in children {
                let name = proc_map.get(child).cloned().unwrap_or_default();
                if name.to_lowercase().contains(keyword) {
                    return true;
                }
                stack.push(*child);
            }
        }
    }
    false
}
