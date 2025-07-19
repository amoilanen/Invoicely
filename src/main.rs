use anyhow::{ Context, Error };
use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use invoicely::renderer::render;
use invoicely::invoice::Invoice;
use printpdf::PdfSaveOptions;

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
    /*let args = Args {
        input: PathBuf::from("examples/1.json"),
        output: PathBuf::from("invoice.pdf"),
    };
    */
    
    let raw_invoice = fs::read_to_string(&args.input)
        .with_context(|| format!("Could not read input file: {}", args.input.display()))?;
    
    let invoice: Invoice = serde_json::from_str(&raw_invoice)
        .context("Could not parse invoice JSON data")?;
    
    let doc = render(&invoice)?;
    
    let output_file = File::create(&args.output)
        .with_context(|| format!("Could not create output file: {}", args.output.display()))?;
    let mut writer = BufWriter::new(output_file);
    
    let mut save_warnings = &mut Vec::new();
    doc.save_writer(&mut writer, &PdfSaveOptions::default(), &mut save_warnings);
    if save_warnings.len() > 0 {
        //println!("Warnings: {:?}", save_warnings);
        //println!("There might had been problems saving PDF to: {}", args.output.display());
    }
    println!("Invoice PDF generated successfully: {}", args.output.display());
    Ok(())
}