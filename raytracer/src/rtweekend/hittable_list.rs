pub mod hittable;

pub struct Hittable_list{
    objects:Vec<hittable>,
};
impl Hittable for Hittable_list{
    pub fn hit(r:&Ray,ray_tmin:f64,ray_tmax:f64) ->(HitRecord,bool) {
        let v=Vec3{e:[0.0,0.0,0.0]};
        let mut rec=HitRecord{
            p:v.clone(),
            normal:v.clone(),
            t:0.0,
            front_face:false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for  object in &objects {
            let (temp_rec,flag)=object.hit(r, ray_tmin, closest_so_far);
            if flag {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        (rec,hit_anything);
    }
}