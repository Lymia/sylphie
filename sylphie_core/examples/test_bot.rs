use static_events::prelude_async::*;
use sylphie_core::prelude::*;
use sylphie_core::commands::ctx::CommandCtx;

#[derive(Module)]
#[module(integral_recursive)]
pub struct TestModule {
    #[module_info] info: ModuleInfo,
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

fn main() {
    SylphieCore::<MyModule>::new("test_bot").start().unwrap();
}