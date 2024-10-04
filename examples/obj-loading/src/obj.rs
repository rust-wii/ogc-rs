pub struct Obj<Data: AsRef<[u8]>> {
    data: Data,
}
#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    InvalidVertex,
    InvalidFace,
}

pub fn from_bytes<Data: AsRef<[u8]>>(data: Data) -> Result<Obj<Data>, Error> {
    //Parse bytes into string
    let Ok(obj) = core::str::from_utf8(data.as_ref()) else {
        return Err(Error::InvalidUtf8);
    };

    let valid_vertex_count = obj
        .lines()
        .filter_map(|line| {
            if line.starts_with("v ") {
                let mut vertices = [0.; 3];
                for (i, pos) in line.splitn(4, ' ').skip(1).enumerate() {
                    let Ok(float) = pos.parse::<f32>() else {
                        return None;
                    };
                    vertices[i] = float;
                }

                Some(vertices)
            } else {
                None
            }
        })
        .count();

    if valid_vertex_count != obj.lines().filter(|line| line.starts_with("v ")).count() {
        return Err(Error::InvalidVertex);
    }

    let valid_face_count = obj
        .lines()
        .filter_map(|line| {
            if line.starts_with("f ") {
                let mut indices: [usize; 3] = [0usize; 3];
                for (i, index) in line.splitn(4, ' ').skip(1).enumerate() {
                    let Ok(idx) = index.parse::<usize>() else {
                        return None;
                    };

                    indices[i] = idx - 1;
                }
                Some(indices)
            } else {
                None
            }
        })
        .count();

    if valid_face_count != obj.lines().filter(|line| line.starts_with("f ")).count() {
        return Err(Error::InvalidFace);
    }

    Ok(Obj { data })
}

impl<Data: AsRef<[u8]>> Obj<Data> {
    pub fn vertices(&self) -> Result<impl Iterator<Item = [f32; 3]> + '_, Error> {
        let Ok(obj) = core::str::from_utf8(self.data.as_ref()) else {
            return Err(Error::InvalidUtf8);
        };

        Ok(obj.lines().filter_map(|line| {
            if line.starts_with("v ") {
                let mut vertices = [0.; 3];
                for (i, pos) in line.splitn(4, ' ').skip(1).enumerate() {
                    let Ok(float) = pos.parse::<f32>() else {
                        return None;
                    };
                    vertices[i] = float;
                }

                Some(vertices)
            } else {
                None
            }
        }))
    }

    pub fn indices(&self) -> Result<impl Iterator<Item = [usize; 3]> + '_, Error> {
        let Ok(obj) = core::str::from_utf8(self.data.as_ref()) else {
            return Err(Error::InvalidUtf8);
        };

        Ok(obj.lines().filter_map(|line| {
            if line.starts_with("f ") {
                let mut indices: [usize; 3] = [0usize; 3];

                for (i, index) in line.splitn(4, ' ').skip(1).enumerate() {
                    //println!("{}", index);
                    let Ok(idx) = index.trim().parse::<usize>() else {
                        return None;
                    };

                    indices[i] = idx - 1;
                }
                Some(indices)
            } else {
                None
            }
        }))
    }
}
