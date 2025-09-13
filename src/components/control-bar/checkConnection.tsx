import { useEffect, useState } from "react";
import { Card } from "../ui/card";
import { Loader, Wifi, WifiOff } from "lucide-react";
import { fetch } from '@tauri-apps/plugin-http';

export default function CheckConnection() {
    const [wifiConnection, setWifiConnection] = useState<boolean>();
    const [isTesting, setIsTesting] = useState<boolean>();

    const testConnection = async () => {
        if (isTesting) {
            setIsTesting(false)
            return
        }
        setIsTesting(true);
        const response = await fetch("https://www.bing.com");
        if (response.status == 200) setWifiConnection(true);
        else setWifiConnection(false);
        setIsTesting(false)
    }

    useEffect(() => { testConnection() }, [])

    return (
        <Card className="flex items-center w-full" onClick={testConnection}>
            {isTesting ? (
                <Loader className="animate-spin duration-10000 ease-linear" />
            ) : (
                wifiConnection ? <Wifi /> : <WifiOff />
            )}
        </Card>
    )
}