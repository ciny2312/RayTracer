pub mod ray;
pub trait Hittable{
    fn hit(&self,r:&Ray,ray_tmin:f64,ray_tmax:f64)->(HitRecord,bool);
}
pub struct HitRecord{
    p:Point3,
    normal:Vec3,
    t:f64,
    front_face:bool,
}
impl HitRecord{
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:Vec3){
        //outward_normal has unit length
        self.front_face=Vec3::dot(&r.dir,&outward_normal)<0.0;
        self.normal=if self.front_face{outward_normal}
        else{outward_normal.fushu()};
    }
}