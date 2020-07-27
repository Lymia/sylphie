use static_events::prelude_async::*;
use sylphie::database::kvs::BaseKvsStore;
use sylphie::database::serializable::SimpleSerialize;
use sylphie::prelude::*;
use sylphie_database::kvs::{PersistentKvsType, TransientKvsType};

#[derive(Module)]
#[module(integral_recursive)]
pub struct TestModule {
    #[module_info] info: ModuleInfo,
    #[submodule] test_store_1: BaseKvsStore<SimpleSerialize<u32>, SimpleSerialize<f32>, PersistentKvsType>,
    #[submodule] test_store_2: BaseKvsStore<SimpleSerialize<u32>, SimpleSerialize<f32>, TransientKvsType>,
}
#[module_impl]
impl TestModule {
    #[command]
    async fn cmd_test_mod(&self, ctx: &CommandCtx<impl Events>) -> Result<()> {
        ctx.respond(&format!("Calling module: {}", self.info.name())).await?;
        Ok(())
    }
}

#[derive(Module)]
#[module(integral_recursive)]
pub struct MyModule {
    #[module_info] info: ModuleInfo,
    #[submodule] submod_a: TestModule,
    #[submodule] submod_b: TestModule,
}

#[module_impl]
impl MyModule {
    #[command]
    async fn cmd_test(&self, ctx: &CommandCtx<impl Events>) -> Result<()> {
        for arg in 0..ctx.args_count() {
            ctx.respond(&format!("Arg #{}: {:?}", arg, ctx.arg(arg).text)).await?;
        }
        Ok(())
    }

    #[command]
    async fn cmd_backtrace(&self, ctx: &CommandCtx<impl Events>) -> Result<()> {
        ctx.respond(&format!("\n\n{:?}", backtrace::Backtrace::new())).await?;
        Ok(())
    }
}

sylphie_root_module! {
    module Test {
        test_bot: MyModule,
    }
}

fn main() {
    SylphieCore::<Test>::new("test_bot").start().unwrap();
}
