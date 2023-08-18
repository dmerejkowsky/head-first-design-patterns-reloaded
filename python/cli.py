class CLI:
    def update(self):
        print("update")

    def run(self):
        print("run")

    def stuff(self):
        print("stuff")

    def coffee(self):
        print("cofee")


cli = CLI()
while True:
    command = input()
    if hasattr(cli, command):
        method = getattr(cli, command)
        method()
    else:
        print("no such command")
