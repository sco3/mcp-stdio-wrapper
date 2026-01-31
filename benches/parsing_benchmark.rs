
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mcp_stdio_wrapper::post_result::PostResult;
use nom::{
    bytes::complete::tag,
    combinator::{map, rest},
    sequence::preceded,
    IResult,
};

// ===================================================================================
// Implementation 1: The original string-based logic
// ===================================================================================

fn process_string_based(res: &PostResult) {
    for line in res.out.lines() {
        let processed_line = if res.sse {
            line.strip_prefix("data: ").map_or("", str::trim)
        } else {
            line.trim()
        };
        if !processed_line.is_empty() {
            // In a real scenario, this would be converted to Bytes.
            // We black_box it to prevent the compiler optimizing the work away.
            black_box(processed_line.to_string());
        }
    }
}

// ===================================================================================
// Implementation 2: The new nom-based logic
// ===================================================================================

/// Trims leading and trailing ASCII whitespace from a byte slice.
fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let from = match bytes.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &[],
    };
    let to = bytes.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &bytes[from..=to]
}

/// A `nom` parser that extracts and trims the content from an SSE line.
fn parse_sse_content(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map(preceded(tag(b"data: "), rest), trim_ascii_whitespace)(input)
}

fn process_nom_based(res: &PostResult) {
    for line_bytes in res.out.as_bytes().split(|&c| c == b'\n') {
        let processed_line = if res.sse {
            match parse_sse_content(line_bytes) {
                Ok((_remaining, content)) => content,
                Err(_) => b"",
            }
        } else {
            trim_ascii_whitespace(line_bytes)
        };

        if !processed_line.is_empty() {
            // In a real scenario, this would be converted to Bytes.
            // We black_box it to prevent the compiler optimizing the work away.
            black_box(processed_line);
        }
    }
}


// ===================================================================================
// Benchmark setup
// ===================================================================================

fn parsing_benchmark(c: &mut Criterion) {
    // Generate a large string to process.
    // It contains a mix of SSE-like lines and plain lines.
    let mut large_string = String::new();
    for i in 0..1000 {
        if i % 2 == 0 {
            large_string.push_str("data: This is some content with a prefix to be stripped and trimmed. \r\n");
        } else {
            large_string.push_str("   This line has no prefix but has leading and trailing whitespace.   \n");
        }
    }

    let sse_result = PostResult {
        session_id: None,
        out: large_string.clone(),
        sse: true,
    };

    let non_sse_result = PostResult {
        session_id: None,
        out: large_string,
        sse: false,
    };

    let mut group = c.benchmark_group("Parsing Logic");

    group.bench_function("SSE/String-based", |b| {
        b.iter(|| process_string_based(black_box(&sse_result)))
    });

    group.bench_function("SSE/Nom-based", |b| {
        b.iter(|| process_nom_based(black_box(&sse_result)))
    });

    group.bench_function("Non-SSE/String-based", |b| {
        b.iter(|| process_string_based(black_box(&non_sse_result)))
    });

    group.bench_function("Non-SSE/Nom-based", |b| {
        b.iter(|| process_nom_based(black_box(&non_sse_result)))
    });

    group.finish();
}

criterion_group!(benches, parsing_benchmark);
criterion_main!(benches);
