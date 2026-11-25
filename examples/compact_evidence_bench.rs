//! Generate or count one high-multiplicity synthetic UMI for memory comparison.
//!
//! Run `generate BAM GTF READS` once, then run `count BAM GTF READS` in fresh
//! processes and measure them with `/usr/bin/time -f '%e %M'`.
use anyhow::{ensure, Context, Result};
use scindere::{counter::ExInCounter, logic::logic_from_name};
use std::{collections::HashSet, io::Cursor};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    ensure!(
        args.len() == 5,
        "usage: compact_evidence_bench generate BAM GTF READS | count BAM GTF READS"
    );
    let reads: usize = args[4].parse()?;
    match args[1].as_str() {
        "generate" => generate(&args[2], &args[3], reads),
        "count" => count(&args[2], &args[3], reads),
        _ => anyhow::bail!("mode must be generate or count"),
    }
}

fn generate(bam: &str, gtf: &str, reads: usize) -> Result<()> {
    ensure!(reads > 0, "READS must be positive");
    std::fs::write(
        gtf,
        concat!(
            "chr1\tbench\texon\t101\t200\t.\t+\t.\tgene_id \"G1\"; transcript_id \"T1\"; gene_name \"Gene1\"; exon_number \"1\";\n",
            "chr1\tbench\texon\t301\t400\t.\t+\t.\tgene_id \"G1\"; transcript_id \"T1\"; gene_name \"Gene1\"; exon_number \"2\";\n",
        ),
    )?;

    use noodles_sam::alignment::io::Write;
    let mut sam = String::from("@HD\tVN:1.6\tSO:coordinate\n@SQ\tSN:chr1\tLN:1000\n");
    for i in 0..reads {
        sam.push_str(&format!(
            "r{i:08}\t0\tchr1\t121\t255\t30M\t*\t0\t0\t*\t*\tCB:Z:a\tMA:Z:u1\tNH:i:1\n"
        ));
    }
    let mut reader = noodles_sam::io::Reader::new(Cursor::new(sam));
    let header = reader.read_header().context("read synthetic SAM header")?;
    let mut writer = noodles_bam::io::Writer::new(std::fs::File::create(bam)?);
    writer.write_header(&header).context("write BAM header")?;
    for record in reader.record_bufs(&header) {
        writer
            .write_alignment_record(&header, &record?)
            .context("write BAM record")?;
    }
    writer.try_finish().context("finish synthetic BAM")?;
    Ok(())
}

fn count(bam: &str, gtf: &str, reads: usize) -> Result<()> {
    let mut counter = ExInCounter::new(
        "bench".into(),
        logic_from_name("Default"),
        Some(HashSet::from(["a".to_string()])),
        "no",
        false,
        "0",
        std::env::temp_dir().to_string_lossy().into_owned(),
        "uint32".into(),
    )?;
    counter.umibarcode_str = "MA".into();
    counter.read_transcriptmodels(gtf)?;
    counter.mark_up_introns(bam, false)?;

    let started = std::time::Instant::now();
    let mut total = 0u64;
    let mut cells = Vec::new();
    counter.count_stream(bam, false, 1, 0, usize::MAX, |layers, keys, _| {
        cells = keys;
        total = layers
            .values()
            .flat_map(|array| array.iter())
            .map(|&count| u64::from(count))
            .sum();
        Ok(())
    })?;
    ensure!(cells == ["|a"] && total == 1, "unexpected benchmark counts");
    println!(
        "reads={reads} cells={} total={total} count_seconds={:.3}",
        cells.len(),
        started.elapsed().as_secs_f64()
    );
    Ok(())
}
