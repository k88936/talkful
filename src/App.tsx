import Theme, {ThemeProvider} from '@jetbrains/ring-ui-built/components/global/theme';
import {BrowserRouter} from 'react-router-dom';
import Group from "@jetbrains/ring-ui-built/components/group/group";
import {AppHeader} from "./components/AppHeader.tsx";

export const App = () => {
    return (
        <ThemeProvider theme={Theme.DARK} className="flex flex-1 bg-(--ring-secondary-background-color)">
            <BrowserRouter>
                <Group className="flex flex-row min-h-screen">
                    <AppHeader/>

                </Group>
            </BrowserRouter>
        </ThemeProvider>
    );
};
