package info.dmerej.homeautomation;

import java.util.stream.IntStream;

public class Remote {
    private final Slot[] onSlots;
    private final Slot[] offSlots;
    private Command undoCommand = new NoOp();

    public Remote() {
        onSlots = new Slot[7];
        offSlots = new Slot[7];
        IntStream.range(0, 7).forEach(i -> {
            onSlots[i] = new Slot();
            offSlots[i] = new Slot();
        });
    }

    public void setCommand(int i, Command onCommand, Command offCommand) {
        onSlots[i].setCommand(onCommand);
        offSlots[i].setCommand(offCommand);
    }

    public void on(int i) {
        var command = onSlots[i].getCommand();
        command.execute();
        undoCommand = offSlots[i].getCommand();
    }

    public void off(int i) {
        var command = offSlots[i].getCommand();
        command.execute();
        undoCommand = onSlots[i].getCommand();
    }

    public void undo() {
        undoCommand.execute();
    }
}
