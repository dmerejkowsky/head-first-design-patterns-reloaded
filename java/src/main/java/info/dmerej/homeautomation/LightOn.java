package info.dmerej.homeautomation;

public class LightOn implements Command {
    private final Light light;

    public LightOn(Light light) {
        this.light = light;
    }

    @Override
    public void execute() {
        light.on();
    }
}
