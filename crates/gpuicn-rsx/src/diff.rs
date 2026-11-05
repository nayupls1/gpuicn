//! A small line-based unified diff (LCS), enough for `gpuicn diff`.

pub fn unified(old: &str, new: &str, context: usize) -> String {
    let a: Vec<&str> = old.lines().collect();
    let b: Vec<&str> = new.lines().collect();

    // LCS table, filled from the end.
    let mut lcs = vec![vec![0u32; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = if a[i] == b[j] { lcs[i + 1][j + 1] + 1 } else { lcs[i + 1][j].max(lcs[i][j + 1]) };
        }
    }

    enum Op<'a> {
        Same(&'a str),
        Del(&'a str),
        Add(&'a str),
    }
    let (mut i, mut j, mut ops) = (0, 0, Vec::new());
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            ops.push(Op::Same(a[i]));
            i += 1;
            j += 1;
        } else if j < b.len() && (i == a.len() || lcs[i][j + 1] >= lcs[i + 1][j]) {
            ops.push(Op::Add(b[j]));
            j += 1;
        } else {
            ops.push(Op::Del(a[i]));
            i += 1;
        }
    }

    // Emit hunks with `context` lines around changes.
    let changed: Vec<bool> = ops.iter().map(|op| !matches!(op, Op::Same(_))).collect();
    let mut out = String::new();
    let mut ix = 0;
    while ix < ops.len() {
        if !changed[ix] {
            ix += 1;
            continue;
        }
        let start = ix.saturating_sub(context);
        let mut end = ix;
        while end < ops.len() {
            if changed[end] {
                end += 1;
                continue;
            }
            let next_change = (end..ops.len()).find(|k| changed[*k]);
            match next_change {
                Some(k) if k - end <= context * 2 => end = k,
                _ => break,
            }
        }
        let end = (end + context).min(ops.len());
        let (old_start, new_start) = ops[..start].iter().fold((1, 1), |(o, n), op| match op {
            Op::Same(_) => (o + 1, n + 1),
            Op::Del(_) => (o + 1, n),
            Op::Add(_) => (o, n + 1),
        });
        let (old_len, new_len) = ops[start..end].iter().fold((0, 0), |(o, n), op| match op {
            Op::Same(_) => (o + 1, n + 1),
            Op::Del(_) => (o + 1, n),
            Op::Add(_) => (o, n + 1),
        });
        out.push_str(&format!("\x1b[36m@@ -{old_start},{old_len} +{new_start},{new_len} @@\x1b[0m\n"));
        for op in &ops[start..end] {
            match op {
                Op::Same(line) => out.push_str(&format!(" {line}\n")),
                Op::Del(line) => out.push_str(&format!("\x1b[31m-{line}\x1b[0m\n")),
                Op::Add(line) => out.push_str(&format!("\x1b[32m+{line}\x1b[0m\n")),
            }
        }
        ix = end;
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn shows_changed_lines() {
        let out = super::unified("a\nb\nc\n", "a\nB\nc\n", 1);
        assert!(out.contains("-b"));
        assert!(out.contains("+B"));
        assert!(out.contains("@@ -1,3 +1,3 @@"));
    }

    #[test]
    fn identical_is_empty() {
        assert!(super::unified("a\nb\n", "a\nb\n", 3).is_empty());
    }
}
