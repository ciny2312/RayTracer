# ACM2023 PPCA : RayTracer
具体要求见：https://github.com/ACMClassCourse-2023/Raytracing-2024
## Advanced features:
* 实现了多线程
* 支持.obj文件：见final scene
* Reduce Contention：在多线程环境中减少 Arc 的使用
* Static Dispatch：
我不是用泛型代替的$Arc<dyn trait>$, 
在写多线程之前，大概是写完第一本书的时候，我感觉自己的代码跑的较慢，
查到了一个用enum代替trait的静态实现方式，
因此接下来一直是用的这种实现，感觉和泛型达到的效果应该是一样的。
* PDF Static Dispatch：减少动态调用的开销
同上。