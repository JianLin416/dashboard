import { useEffect, useState } from "react";
import { Card } from "../ui/card";
import { Moon, Sun } from "lucide-react";

export default function SelectTheme() {
    const [isDark, setIsDark] = useState<boolean>(true);

    const changeTheme = async () => {
        const newTheme = !isDark;
        setIsDark(newTheme);

        if (newTheme)
            document.documentElement.classList.add("dark");
        else
            document.documentElement.classList.remove("dark");
    }

    useEffect(() => {
        document.documentElement.classList.add("dark");
    }, [])

    return (
        <Card className="flex items-center w-full" onClick={changeTheme}>
            {isDark ? <Moon /> : <Sun />}
        </Card>
    )
}