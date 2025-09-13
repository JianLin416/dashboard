import {
    Card,
    CardContent,
    CardDescription
} from "@/components/ui/card"

export default function ThrottlePos({
    throttlePos
}: { throttlePos: string }) {
    return (
        <Card className="flex-1 relative overflow-hidden">
            <CardContent className="h-full flex flex-col">
                <CardDescription>节气门位置</CardDescription>
                <div className="flex items-end place-content-center my-auto gap-2">
                    <div className="text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl">
                        {throttlePos}
                    </div>
                    <div className="text-md xl:text-lg">%</div>
                </div>
            </CardContent>
        </Card>
    )
}