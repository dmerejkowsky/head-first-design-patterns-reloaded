package info.dmerej.homeautomation;

public class CeilingFanOff implements Command {
    private final CeilingFan ceilingFan;

    public CeilingFanOff(CeilingFan ceilingFan) {
        this.ceilingFan = ceilingFan;
    }

    @Override
    public void execute() {
        this.ceilingFan.off();
    }
}
