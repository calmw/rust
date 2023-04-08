#### 介绍

    使用unsafe关键字并不表示此段代码具有内在的危险性，比如它不允许你绕过rust的借用检查器。在unsafe块允许的各种功能中，某些功能要比其他一些功能更加难以验证。就来说std::mem::transmute()函数就是rust语言中最不安全的功能之一，它已经没有任何类型的安全性可言。因此你要在使用它之前，请先研究有没有其他替代方案。
    unsafe块允许深入字节内部