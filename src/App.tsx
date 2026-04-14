import Theme, {ThemeProvider} from '@jetbrains/ring-ui-built/components/global/theme';
import {BrowserRouter, Navigate, Route, Routes} from 'react-router-dom';
import Group from "@jetbrains/ring-ui-built/components/group/group";
import {AppHeader} from "@/components/AppHeader.tsx";
import {ErrorBanners} from '@/features/system-errors/components/ErrorBanners.tsx';
import {useErrorEventBanners} from '@/features/system-errors/hooks/useErrorEventBanners.ts';
import {SettingsPage} from "@/pages/settings/SettingsPage.tsx";
import {StatisticsPage} from "@/pages/statistics/StatisticsPage.tsx";

const AppRoutes = () => {
    return (
        <Routes>
            <Route path="/" element={<Navigate to="/settings" replace/>}/>
            <Route path="/statistics" element={<StatisticsPage/>}/>
            <Route path="/settings" element={<SettingsPage/>}/>
        </Routes>
    );
};
export const App = () => {
    const {errors, dismissError} = useErrorEventBanners();

    return (
        <ThemeProvider theme={Theme.DARK} className="flex flex-1 bg-(--ring-secondary-background-color)">
            <BrowserRouter>
                <Group className="flex min-h-screen w-full flex-row">
                    <AppHeader/>
                    <Group className="flex w-full flex-1 p-6">
                        <Group className="flex w-full flex-1 flex-col">
                            <ErrorBanners errors={errors} onDismiss={dismissError}/>
                            <AppRoutes/>
                        </Group>
                    </Group>
                </Group>
            </BrowserRouter>
        </ThemeProvider>
    );
};
