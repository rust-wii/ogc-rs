use core::str::FromStr;

pub struct Obj<Data: AsRef<[u8]>> {
    data: Data,
}
#[derive(Debug)]
pub enum Error {
    InvalidUtf8,
    InvalidVertex,
    InvalidFace,
    InvalidNormal,
    InvalidTexcoord,
}

pub fn from_bytes<Data: AsRef<[u8]>>(data: Data) -> Result<Obj<Data>, Error> {
    //Parse bytes into string
    let Ok(obj) = core::str::from_utf8(data.as_ref()) else {
        return Err(Error::InvalidUtf8);
    };

    let valid_face_count = obj
        .lines()
        .filter_map(|line| {
            if line.starts_with("f ") {
                let mut indices: [usize; 3] = [0usize; 3];
                for (i, index) in line.splitn(4, ' ').skip(1).enumerate() {
                    let has_slash = index.contains('/');

                    let idx: Option<usize> = if has_slash {
                        let mut split = index.split('/');

                        split
                            .next()
                            .and_then(|idx| idx.parse::<usize>().ok())
                            .map(|idx| idx - 1)
                    } else {
                        index.parse::<usize>().ok().map(|idx| idx - 1)
                    };

                    idx?;

                    indices[i] = idx.expect("idx shouldn't be None");
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

    let valid_vertex_position_count = obj_iter::<f32>(obj, "v ").count();
    let valid_vertex_normal_count = obj_iter::<f32>(obj, "vn ").count();
    let valid_vertex_texcoord_count = obj_iter2::<f32>(obj, "vt ").count();

    if valid_vertex_position_count != obj.lines().filter(|line| line.starts_with("v ")).count() {
        return Err(Error::InvalidVertex);
    }

    if valid_vertex_normal_count != obj.lines().filter(|line| line.starts_with("vn ")).count() {
        return Err(Error::InvalidNormal);
    }

    if valid_vertex_texcoord_count != obj.lines().filter(|line| line.starts_with("vt ")).count() {
        return Err(Error::InvalidTexcoord);
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

    pub fn indices(
        &self,
    ) -> Result<impl Iterator<Item = [(usize, Option<usize>, Option<usize>); 3]> + '_, Error> {
        let Ok(obj) = core::str::from_utf8(self.data.as_ref()) else {
            return Err(Error::InvalidUtf8);
        };

        Ok(obj.lines().filter_map(|line| {
            if line.starts_with("f ") {
                let mut indices = [(0usize, None, None); 3];
                for (i, index) in line.splitn(4, ' ').skip(1).enumerate() {
                    let has_slash = index.contains('/');

                    let idx = if has_slash {
                        let mut split = index.split('/');

                        let pos_idx = split
                            .next()
                            .and_then(|pos_idx| pos_idx.parse::<usize>().ok());

                        let tex_idx = split
                            .next()
                            .and_then(|pos_idx| pos_idx.parse::<usize>().ok());

                        let nrm_idx = split
                            .next()
                            .and_then(|pos_idx| pos_idx.parse::<usize>().ok());
                        (
                            pos_idx
                                .map(|idx| idx - 1)
                                .expect("pos_idx shouldnt be none at this point"),
                            tex_idx.map(|idx| idx - 1),
                            nrm_idx.map(|idx| idx - 1),
                        )
                    } else {
                        let idx = index.parse::<usize>().ok();

                        idx?;

                        (
                            idx.map(|idx| idx - 1)
                                .expect("pos_idx shouldn't be none at this point"),
                            None,
                            None,
                        )
                    };

                    indices[i] = idx;
                }
                Some(indices)
            } else {
                None
            }
        }))
    }

    pub fn normals(&self) -> Result<impl Iterator<Item = [f32; 3]> + '_, Error> {
        let Ok(obj) = core::str::from_utf8(self.data.as_ref()) else {
            return Err(Error::InvalidUtf8);
        };
        Ok(obj_iter(obj, "vn "))
    }

    pub fn texcoords(&self) -> Result<impl Iterator<Item = [f32; 2]> + '_, Error> {
        let Ok(obj) = core::str::from_utf8(self.data.as_ref()) else {
            return Err(Error::InvalidUtf8);
        };
        Ok(obj_iter2(obj, "vt "))
    }
}

fn obj_iter<'a, T: Default + Copy + FromStr>(
    str: &'a str,
    prefix: &'a str,
) -> impl Iterator<Item = [T; 3]> + 'a {
    str.lines().filter_map(move |line| {
        if line.starts_with(prefix) {
            let mut array = [T::default(); 3];
            for (i, t) in line.splitn(4, ' ').skip(1).enumerate() {
                let Ok(t) = t.parse::<T>() else {
                    return None;
                };
                array[i] = t;
            }
            Some(array)
        } else {
            None
        }
    })
}

fn obj_iter2<'a, T: Default + Copy + FromStr>(
    str: &'a str,
    prefix: &'a str,
) -> impl Iterator<Item = [T; 2]> + 'a {
    str.lines().filter_map(move |line| {
        if line.starts_with(prefix) {
            let mut array = [T::default(); 2];
            for (i, t) in line.splitn(3, ' ').skip(1).enumerate() {
                let Ok(t) = t.parse::<T>() else {
                    return None;
                };
                array[i] = t;
            }
            Some(array)
        } else {
            None
        }
    })
}
