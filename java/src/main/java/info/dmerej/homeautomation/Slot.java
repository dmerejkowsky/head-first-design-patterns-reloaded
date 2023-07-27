package info.dmerej.homeautomation;

public class Slot {
    private Command command;

    public Slot() {
        this.command = new NoOp();
    }

    public Command getCommand() {
        return command;
    }

    public void setCommand(Command command) {
        this.command = command;
    }
}
