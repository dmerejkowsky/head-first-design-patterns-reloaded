package info.dmerej.homeautomation;

public class TV {
    int volume = 5;

    public void on() {
        System.out.println("Turning TV on");
    }

    public void off() {
        System.out.println("Turning TV off");
    }

    public void setVolume(int volume) {
        System.out.format("Turning TV volume to %d\n");
        this.volume = volume;
    }
}
