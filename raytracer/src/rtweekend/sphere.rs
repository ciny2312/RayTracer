pub mod hittable;
//pub mod vec3;
pub struct Sphere{
    center:Point3,
    radius:f64,
}
impl Hittable for Sphere{
    pub fn hit (&self,r:&Ray,ray_tmin:f64,ray_tmax:f64)->(HitRecord,bool){
        let v=Vec3{e:[0.0,0.0,0.0]};
        let mut rec=HitRecord{
            p:v.clone(),
            normal:v.clone(),
            t:0.0,
            front_face:false,
        };
        let oc = Vec3::del(&self.center, &r.ori);
        let a = r.dir.sq_length();
        let h = Vec3::dot(&r.dir, &oc);
        let c = oc.sq_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return (rec,false);
        }
        let sqrtd=discriminant.sqrt();
        let mut root=(h-sqrtd)/a;
        if root<=ray_tmin||root>=ray_tmax{
            root =(h+sqrtd)/a;
            if root<=ray_tmin||root>=ray_tmax{
                return (rec,false);
            }
        }
        rec.t=root;
        rec.p=r.at(rec.t);
        let outward_normal=Vec3::div(&Vec3::del(&rec.p,&self.center),self.radius);
        rec.set_face_normal(&r,outward_normal);
        (rec,true)
    }
}