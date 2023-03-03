// 定义特征trait
// 对于Playable必须用pub公开来修饰，这样别的地方use media 才可以使用Playable trait
// 在默认情况下，特征是私有的。要能够被其他模块或其他软件包调用，它们需要被声明为公有的。
pub trait Playable {
    // 实例方法定义 这里&self表示借用当前实现的实例对象
    fn play(&self);

    // 关联方法，相当于其他语言的静态方法，可以不需要实例，直接执行
    fn pause() {
        println!("paused");
    }
}
