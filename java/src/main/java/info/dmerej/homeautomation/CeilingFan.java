package info.dmerej.homeautomation;

public class CeilingFan {
    private int speed = 0;

    public void high() {
        setSpeed(3);
    }

    public void medium() {
        setSpeed(2);
    }

    public void low() {
        setSpeed(1);
    }

    public void off() {
        setSpeed(0);
    }

    public int getSpeed() {
        return speed;
    }

    private void setSpeed(int value) {
        System.out.printf("Ceiling at %d speed\n", speed);
        speed = value;
    }
}
