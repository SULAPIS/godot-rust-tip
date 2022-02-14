use gdnative::api::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
//add this
#[register_with(Self::register_signals)]
pub struct Agent;
impl Agent {
    fn new(_owner: &Node) -> Self {
        Agent
    }
}

#[methods]
impl Agent {
    //register signal function
    fn register_signals(builder: &ClassBuilder<Self>) {
        //register signal with 0 argument
        builder.add_signal(Signal {
            name: "reset",
            args: &[],
        });

        //register signal with multiple arguments
        builder.add_signal(Signal {
            name: "step",
            args: &[
                SignalArgument {
                    name: "arg1",
                    default: Variant::from_i64(2),
                    export_info: ExportInfo::new(VariantType::I64),
                    usage: PropertyUsage::empty(),
                },
                SignalArgument {
                    name: "arg2",
                    default: Variant::from_str("default"),
                    export_info: ExportInfo::new(VariantType::GodotString),
                    usage: PropertyUsage::empty(),
                },
            ],
        })
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {
        //connect signal,there use Timer's timeout signal
        let node = &mut owner.get_node("Env/Timer").unwrap();
        let node = unsafe { node.assume_safe() };
        node.connect(
            "timeout",
            owner,
            "rust_timeout",
            VariantArray::new_shared(),
            0,
        )
        .unwrap();
    }

    #[export]
    fn _process(&self, owner: &Node, _delta: f64) {
        //emit_signal
        owner.emit_signal(
            "step",
            &[Variant::from_i64(2), Variant::from_str("sulapis")],
        );
    }

    #[export]
    fn rust_timeout(&self, _owner: &Node) {
        godot_print!("timeout");
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Agent>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
