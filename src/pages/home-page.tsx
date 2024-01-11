import {cn} from "@/lib/utils.ts";

const HomePage = () => {
    return (
        <div className={cn('w-full h-full grid place-content-center gap-2')}>
            <h1 className={cn('text-4xl')}>s-tools</h1>
            <p className={cn('text-xl')}>A collection of tools for for your daily use.</p>
            <p className={cn('text-base')}>Build by solidSpoon with ❤️</p>
        </div>
    );
};

export default HomePage;