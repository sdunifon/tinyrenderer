use super::*;
use std::mem;

enum RenderStyle {
    Wireframe,
    Filled,
}

pub trait Fillable<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized + Drawable<H, W>,
{
    fn vertices(&self) -> [Vertex; 3];

    fn fill(&self, image: &mut Image<H, W>, px: Px) {
        // // sort the vertices, v0, t1, t2 lower−to−upper (bubblesort yay!)
        // if v0.y>v1.y {std::swap(v0, t1)};
        // if v0.y>v2.y {std::swap(v0, t2)};
        // if v1.y>v2.y {std::swap(v1, t2)};
        let vn = self.vertices();
        let (v0, v1, v2): (Vertex, Vertex, Vertex) = (vn[0], vn[1], vn[2]);
        let total_height: u32 = v2.y - v0.y;

        {
            let mut y = v0.y;

            while y <= v1.y {
                let segment_height = v1.y - v0.y + 1;

                let alpha: f64 = (y - v0.y) as f64 / total_height as f64;
                let beta: f64 = (y - v0.y) as f64 / segment_height as f64;

                let a = v0 + (v2 - v0) * alpha;
                let b = v0 + (v1 - v0) * beta;
                //if a.x > b.x {
                //    //double check this is working
                //    mem::swap(&mut a, &mut b);
                //}

                // let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };
                swap_vars!(a.x > b.x, a, b);

                {
                    let j: usize = a.x as usize;
                    while j <= b.x as usize {
                        image.set(Pt(j, y as usize), px)
                    }
                }
                y += 1;
            }
        }

        // for (int y=v0.y; y<=v1.y; y++) {
        //     int segment_height = v1.y-v0.y+1;
        //     float alpha = (float)(y-v0.y)/total_height;
        //     float beta  = (float)(y-v0.y)/segment_height; // be careful with divisions by zero
        //     Vec2i A = v0 + (v2-v0)*alpha;
        //     Vec2i B = v0 + (v1-v0)*beta;
        //     if (A.x>B.x) std::swap(A, B);
        //     for (int j=A.x; j<=B.x; j++) {
        //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
        //     }
        // }

        {
            let mut y = v1.y;

            while y <= v2.y {
                let segment_height = v2.y - v1.y + 1;

                let alpha: f64 = (y - v0.y) as f64 / total_height as f64;
                let beta: f64 = (y - v1.y) as f64 / segment_height as f64;

                let a = v0 + (v2 - v0) * alpha;
                let b = v1 + (v2 - v1) * beta;

                swap_vars!(a.x > b.x, a, b);

                {
                    let j: usize = a.x as usize;
                    while j <= b.x as usize {
                        image.set(Pt(j, y as usize), px)
                    }
                }
                y += 1;
            }
        }

        // for (int y=v1.y; y<=t2.y; y++) {
        //     int segment_height =  v2.y-t1.y+1;
        //     float alpha = (float)(y-v0.y)/total_height;
        //     float beta  = (float)(y-v1.y)/segment_height; // be careful with divisions by zero
        //     Vec2i A = v0 + (t2-t0)*alpha;
        //     Vec2i B = v1 + (t2-t1)*beta;
        //     if (A.x>B.x) std::swap(A, B);
        //     for (int j=A.x; j<=B.x; j++) {
        //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
        //     }
        // }
    }

    fn sorted_verticies(&self) -> (Vertex, Vertex, Vertex) {
        let mut va = self.vertices();
        va.sort_by(|a, b| a.y.cmp(&b.y));

        (va[0], va[1], va[2])
    }
}
