use rand::prelude::*;

struct random_box<R: Rng>{
    low: f32,
    up: f32,
    rect: (f32, f32, f32, f32),
    rng: R,
    iou_range: (f32, f32),
    dx_range: (f32, f32),
    dy_range:(f32, f32),
    dw_range:(f32, f32),
    dh_range: (f32, f32),
}

impl<R: Rng> random_box<R>{
    fn new(low: f32, up: f32, rect: (f32, f32, f32, f32), rng: R) -> Self{
        let range_init = (999.0, -1.0);
        random_box{low, up, rect, rng, iou_range: range_init, dx_range: range_init,
        dy_range: range_init, dw_range: range_init, dh_range: range_init}
    }

    fn random(&mut self) -> (f32, f32, f32, f32){
        let range = self.up - self.low;
        (self.rng.gen::<f32>() * range + self.low, self.rng.gen::<f32>() * range + self.low,
         self.rng.gen::<f32>() * range + self.low, self.rng.gen::<f32>() * range + self.low)
    }

    fn get_box(&mut self){
        let mut box_random;
        loop{
            box_random = self.random();
            match self.compute_iou(&mut box_random){
                Some(n) if n > 0.8 => break,
                Some(_) | None => {},
            }
        }

        let (w, h, c_x, c_y) = get_whxy(&box_random);
        let (r_w, r_h, rc_x, rc_y) = get_whxy(&self.rect);

        let dx = (rc_x - c_x) / w;
        let dy = (rc_y - c_y) / h;
        let dw = (r_w / w).ln();
        let dh = (r_h / h).ln();

        write_to_range(dx, &mut self.dx_range);
        write_to_range(dy, &mut self.dy_range);
        write_to_range(dw, &mut self.dw_range);
        write_to_range(dh, &mut self.dh_range);
    }

    fn compute_iou(&mut self, bbox: &mut (f32, f32, f32, f32)) -> Option<f32>{
        if bbox.0 > bbox.2 {
            let tmp = bbox.0;
            bbox.0 = bbox.2;
            bbox.2 = tmp;
        }
        if bbox.1 > bbox.3{
            let tmp = bbox.1;
            bbox.1 = bbox.3;
            bbox.3 = tmp;
        }
        let rect_area = (self.rect.2 - self.rect.1) * (self.rect.3 - self.rect.1);
        let box_area = (bbox.2 - bbox.0) * (bbox.3 - bbox.1);

        let x1 = max(bbox.0, self.rect.0);
        let y1 = max(bbox.1, self.rect.1);

        let x2 = min(bbox.2, self.rect.2);
        let y2 = min(bbox.3, self.rect.3);

        if x2 - x1 <= 0.0 || y2 - y1 <= 0.0{
            return None
        }

        let intersection = (x2 - x1) * (y2 - y1);
        let iou = intersection / (rect_area + box_area - intersection);
        write_to_range(iou, &mut self.iou_range);
        Some(iou)
    }
}

fn max(a: f32, b: f32) -> f32{
        if a > b{a}else{b}
    }
fn min(a: f32, b: f32) -> f32{
    if a > b{b}else{a}
}

fn get_whxy(bbox: &(f32, f32, f32, f32)) -> (f32, f32, f32, f32){
    let h = bbox.3 - bbox.1;
    let w = bbox.2 - bbox.0;
    let cent_y = bbox.1 + 0.5 * h;
    let cent_x = bbox.0 + 0.5 * w;
    (w, h, cent_x, cent_y)
}

fn write_to_range(value: f32, range: &mut (f32, f32)){
    if value < range.0{
        range.0 = value;
    }else if value > range.1{
        range.1 = value;
    }
}


fn main() {
    let rng = rand::thread_rng();
    let rect1 = (0.0, 0.0, 10.0, 10.0);
    let mut rng_gen = random_box::new(-10.0, 20.0, rect1, rng);
    for i in 0..4000000{
        if i % 50000 == 0{
            println!("steps: {}", i);
        }
        rng_gen.get_box();
    }
    println!("iou_range: {:?}", rng_gen.iou_range);
    println!("dx_range: {:?}", rng_gen.dx_range);
    println!("dy_range: {:?}", rng_gen.dy_range);
    println!("dw_range: {:?}", rng_gen.dw_range);
    println!("dh_range: {:?}", rng_gen.dh_range);
}
