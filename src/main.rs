use printpdf::*;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::convert::From;

fn main() {
    // Take input from user
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut subjects = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks obtained:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut subjects).unwrap();

    // Parse marks and subject count
    let total_marks: f64 = total_marks.trim().parse().unwrap_or(0.0);
    let subjects: f64 = subjects.trim().parse().unwrap_or(1.0);

    // Calculate percentage and grade
    let percentage = total_marks / (subjects * 100.0) * 100.0;
    let grade = if percentage >= 90.0 {
        "A+"
    } else if percentage >= 75.0 {
        "A"
    } else if percentage >= 60.0 {
        "B"
    } else if percentage >= 50.0 {
        "C"
    } else {
        "D"
    };

    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new("Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load a font
    let font = doc
        .add_external_font(File::open("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf").unwrap())
        .unwrap();

    // Write text to the PDF
    current_layer.use_text(
        format!("Report Card for {}", name.trim()),
        24.0,
        Mm(20.0),
        Mm(270.0),
        &font,
    );

    current_layer.use_text(
        format!("Total Marks: {}", total_marks),
        18.0,
        Mm(20.0),
        Mm(250.0),
        &font,
    );

    current_layer.use_text(
        format!("Number of Subjects: {}", subjects),
        18.0,
        Mm(20.0),
        Mm(240.0),
        &font,
    );

    current_layer.use_text(
        format!("Percentage: {:.2}%", percentage),
        18.0,
        Mm(20.0),
        Mm(230.0),
        &font,
    );

    current_layer.use_text(
        format!("Grade: {}", grade),
        18.0,
        Mm(20.0),
        Mm(220.0),
        &font,
    );

    // Save PDF to file
    let file = File::create("report_card.pdf").unwrap();
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).unwrap();

    println!("✅ Report card generated successfully as 'report_card.pdf'");
}






