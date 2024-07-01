const empty    = interval(+INF, -INF);
const universe = interval(-INF, +INF);
struct interval{
    min:f64,
    max:f64,
}
impl interval{
    fn size(&self)->f64{
        self.max-self.min
    }
    fn contains(&self,x:f64)->bool{
        min<=x&&x<=max
    }
    fn surrounds(&self,x:f64)->bool{
        min<x&&x<max
    }
}