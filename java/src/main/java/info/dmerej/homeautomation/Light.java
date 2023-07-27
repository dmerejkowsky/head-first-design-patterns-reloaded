package info.dmerej.homeautomation;

public class Light {
    private final String location;

    public Light(String location) {
        this.location = location;
    }

    public void on() {
        System.out.format("Turning light in %s on\n", location);
    }

    public void off() {
        System.out.format("Turning light in %s off\n", location);
    }
}
