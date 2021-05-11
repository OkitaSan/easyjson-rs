
pub mod lexer;
pub mod parser;
pub mod useful_kt_extensions {
    /// You can see how to use `let` and `also` function in kotlin
    ///
    /// Note that to correspond with the ownership and lifetime rule of rust,
    /// A number of `let` and `also` function with proper ownership and lifetime rule is provided.
    pub trait KotlinScopeFunction {
        fn kotlin_let_ref<'a, R, F>(&'a self, block: F) -> R
        where
            F: FnOnce(&'a Self) -> R,
            R: 'a;
        fn kotlin_let_mut_ref<'a, R, F>(&'a mut self, block: F) -> R
        where
            F: FnOnce(&'a mut Self) -> R;
        fn kotlin_let<R, F>(self, block: F) -> R
        where
            F: FnOnce(Self) -> R,
            Self: Sized;
        fn kotlin_also_ref<'a,F>(&self, block:F) -> &Self
            where F:FnOnce(&Self) -> ();
        fn kotlin_also_mut_ref<'a,F>(&mut self,block:F) -> &mut Self
            where F:FnOnce(&mut Self) -> ();
        fn kotlin_also_by_ref<F>(self,block:F) -> Self
            where F:FnOnce(&Self) -> ();
        fn kotlin_also_by_mut_ref<F>(self,block:F) -> Self
            where F:FnOnce(&mut Self) -> ();
    }
    impl<T> KotlinScopeFunction for T {
        #[inline]
        fn kotlin_let_ref<'a, R, F>(&'a self, block: F) -> R
        where
            F: FnOnce(&'a Self) -> R,
            R: 'a,
        {
            return block(self);
        }
        #[inline]
        fn kotlin_let_mut_ref<'a, R, F>(&'a mut self, block: F) -> R
        where
            F: FnOnce(&'a mut Self) -> R,
        {
            block(self)
        }
        #[inline]
        fn kotlin_let<R, F>(self, block: F) -> R
        where
            F: FnOnce(Self) -> R,
            Self: Sized,
        {
            block(self)
        }
        #[inline]
        fn kotlin_also_ref<'a, F>(&self, block: F) -> &Self where F: FnOnce(&Self) -> () {
            block(self);
            self
        }
        #[inline]
        fn kotlin_also_mut_ref<'a, F>(&mut self, block: F) -> &mut Self where F: FnOnce(&mut Self) -> () {
            block(self);
            self
        }

        fn kotlin_also_by_ref<F>(self,block:F) -> Self
            where F:FnOnce(&Self) -> () {
            block(&self);
            self
        }

        fn kotlin_also_by_mut_ref<F>(self,block:F) -> Self
            where F:FnOnce(&mut Self) -> () {
            let mut k = self;
            block(&mut k);
            k
        }
    }
}
