use anyhow::{ Context, Error };
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use invoice_generator::renderer::render;
use invoice_generator::invoice::Invoice;

#[derive(Parser)]
#[command(name = "invoice-generator")]
#[command(about = "Generate PDF invoices from JSON data")]
struct Args {
    /// Input JSON file containing invoice data
    #[arg(short, long)]
    input: PathBuf,
    
    /// Output PDF file path
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    let raw_invoice = fs::read_to_string(&args.input)
        .with_context(|| format!("Could not read input file: {}", args.input.display()))?;
    
    let invoice: Invoice = serde_json::from_str(&raw_invoice)
        .context("Could not parse invoice JSON data")?;
    
    let doc = render(&invoice)?;
    
    let output_file = File::create(&args.output)
        .with_context(|| format!("Could not create output file: {}", args.output.display()))?;
    let mut writer = BufWriter::new(output_file);
    
    doc.save(&mut writer)
        .with_context(|| format!("Could not save PDF to: {}", args.output.display()))?;
    
    println!("Invoice PDF generated successfully: {}", args.output.display());
    Ok(())
}