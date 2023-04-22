use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};

pub struct Mesh {
    pub faces: Vec<u32>,
    pub verts: Vec<f32>,
}

impl Mesh {
    // Parse a VTK file into a Mesh struct
    fn from_vtk_file<P: AsRef<Path>>(file_path: P) -> Result<Mesh, Box<dyn std::error::Error>> {
        let file = fs::File::open(file_path)?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        // Find the start and end indices of the points section
        let start_points = lines.iter().position(|l| l.starts_with("POINTS")).unwrap();
        let sp: Vec<&str> = lines[start_points].split_whitespace().collect();
        let end_points = sp[1].parse()?;
        // dbg!(&start_points, &end_points);

        // Parse the vertices
        let verts: Vec<f32> = lines[start_points + 1..end_points]
            .iter()
            .flat_map(|l| {
                l.split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<f32>().unwrap())
            })
            .collect();

        // Find the start and end indices of the polygons section
        // Find the start and end indices of the points section
        let start_polys = lines
            .iter()
            .position(|l| l.starts_with("POLYGONS"))
            .unwrap();

        let sp: Vec<&str> = lines[start_polys].split_whitespace().collect();
        let end_points: usize = sp[1].parse()?;

        // Parse the faces
        let faces: Vec<u32> = lines[start_polys + 1..end_points]
            .iter()
            .flat_map(|l| {
                let parts: Vec<u32> = l
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                parts[1..].iter().copied().collect::<Vec<u32>>()
            })
            .collect();

        Ok(Mesh { verts, faces })
    }
}

#[cfg(test)]
mod tests {
    use crate::Mesh;
    use std::path::PathBuf;

    #[test]
    fn test_line_reader_from_file() {
        let path = PathBuf::from("sample.vtk");
        let mesh = Mesh::from_vtk_file(&path).unwrap();
        dbg!(mesh.verts.len() / 3);
        dbg!(mesh.faces.len() / 3);
    }
}
