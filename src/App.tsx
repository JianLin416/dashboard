import { useEffect, useState } from "react";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import ThrottlePos from "./components/state-readers/throttlePos";
import Rpm from "./components/state-readers/rpm";
import Speed from "./components/state-readers/speed";
import { Card, CardContent } from "./components/ui/card";
import OutsideTemp from "./components/state-readers/outsideTemp";
import IntakeTemp from "./components/state-readers/intakeTemp";
import CoolerTemp from "./components/state-readers/coolerTemp";
import SelectTheme from "./components/control-bar/selectTheme";
import CheckConnection from "./components/control-bar/checkConnection";
import Transport from "./components/control-bar/transport";

function App() {

    const [data, setData] = useState<Record<string, string>>({});

    useEffect(() => {
        const unlisten = listen("serial-data", (event) => {
            setData(event.payload as Record<string, string>);
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    return (
        <div className="flex h-screen gap-3 xl:gap-6 flex-row-reverse p-5 xl:p-15">
            <div className="flex flex-col flex-1 justify-between gap-3 xl:gap-6">
                <Rpm rpm={data["rpm"]} />
                <Speed speed={data["speed"]} />
                <ThrottlePos throttlePos={data["throttlePos"]} />
            </div>
            <Card className="flex-1">
                <CardContent className="flex flex-col justify-between h-full gap-2 xl:gap-4">
                    <OutsideTemp outsideTemp={data["outsideTemp"]} />
                    <CoolerTemp coolerTemp={data["coolerTemp"]} />
                    <IntakeTemp intakeTemp={data["intakeTemp"]} />
                </CardContent>
            </Card>
            <div className="w-1/12 flex flex-col gap-2 xl:gap-4 items-center justify-between">
                <SelectTheme />
                <Transport />
                <CheckConnection />
            </div>
        </div>
    );
}

export default App;
