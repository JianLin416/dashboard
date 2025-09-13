import { Card, CardContent, CardDescription } from "@/components/ui/card"
import { Rabbit } from "lucide-react"

export default function Rpm({
    rpm
}: { rpm: string }) {

    return (
        <Card className="flex-1 relative overflow-hidden">
            <CardContent className="h-full flex flex-col">
                <CardDescription>
                    <Rabbit />
                </CardDescription>
                <div className="flex items-end place-content-center relative z-10 my-auto gap-2">
                    <div className="text-3xl md:text-4xl lg:text-5xl xl:text-6xl 2xl:text-7xl">
                        {rpm}
                    </div>
                </div>
            </CardContent>
        </Card>
    )
}