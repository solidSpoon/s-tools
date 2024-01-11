import {Link, useLocation} from "react-router-dom";
import {Button} from "@/components/ui/button.tsx";
import {cn} from "@/lib/utils.ts";

const LinkButton = ({to, children}: {
    to: string,
    children: React.ReactNode,
}) => {
    const location = useLocation();
    return (
        <Button
            className={cn(
                "w-full justify-start",
            )}
            variant={"ghost"}
            asChild>
            <Link
                className={cn(
                    location.pathname === to
                        ? "bg-muted hover:bg-muted"
                        : "hover:bg-transparent hover:underline",
                )}
                to={to}>
                {children}
            </Link>
        </Button>
    );
}
export default LinkButton;