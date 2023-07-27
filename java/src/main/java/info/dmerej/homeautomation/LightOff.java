package info.dmerej.homeautomation;

public class LightOff implements Command {
    private final Light light;

    public LightOff(Light light) {
        this.light = light;
    }

    @Override
    public void execute() {
        light.off();
    }
}
