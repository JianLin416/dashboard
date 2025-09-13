
import { CardDescription } from "@/components/ui/card";
import { Waves } from "lucide-react";

export default function CoolerTemp({
    coolerTemp
}: {
    coolerTemp: string
}) {
    return (
        <div className={"flex flex-col h-1/3"}>
            <CardDescription>
                <div className="flex gap-1 items-center">
                    <Waves />
                    <p>冷却液温度</p>
                </div>
            </CardDescription>
            <div className="flex items-end place-content-center gap-2 my-auto">
                <div className="text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl">
                    {coolerTemp}
                </div>
                <div className="text-md xl:text-lg">℃</div>
            </div>
        </div>
    )
}
