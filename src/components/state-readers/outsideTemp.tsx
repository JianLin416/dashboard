import { CardDescription } from "@/components/ui/card";
import { Sun } from "lucide-react";

export default function OutsideTemp({
    outsideTemp
}: {
    outsideTemp: string
}) {
    return (
        <div className={"flex flex-col flex-1"}>
            <CardDescription>
                <div className="flex gap-1 items-center">
                    <Sun />
                    <p>今日气温</p>
                </div>
            </CardDescription>
            <div className="flex items-end place-content-center my-auto gap-2">
                <div className="text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl">
                    {outsideTemp}
                </div>
                <div className="text-md xl:text-lg">℃</div>
            </div>
        </div>
    )
}