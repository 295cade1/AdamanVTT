Rework network events to not use Bevy's event system and instead just use a resource.
This way the network events can wait for a good connection / allowance for networked events to be sent before sending.
Currently they must be sent on the frame they are created.