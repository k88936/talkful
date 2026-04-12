import {useLocation, useNavigate} from 'react-router-dom';

import Header, {Logo, Tray, HeaderIcon} from '@jetbrains/ring-ui-built/components/header/header';
import Link from '@jetbrains/ring-ui-built/components/link/link';
import Links from '@jetbrains/ring-ui-built/components/header/links';
import analyticsIcon from "@jetbrains/icons/analytics-20px"
import settingsIcon from '@jetbrains/icons/settings-20px';
import bellIcon from '@jetbrains/icons/bell-20px';
import helpIcon from '@jetbrains/icons/help-20px';
import {Color} from "@jetbrains/ring-ui-built/components/icon";

import LOGO from '@/assets/logo.svg?raw';

export const AppHeader = () => {
    const location = useLocation();
    const navigate = useNavigate();

    const isActive = (path: string) => location.pathname.startsWith(path);

    return (
        <Header
            vertical>
            <Link className="items-center no-underline">
                <Logo glyph={LOGO} color={Color.MAGENTA}/>
            </Link>
            <Links>
                <HeaderIcon
                    icon={analyticsIcon}
                    title="statistics"
                    active={false}
                    onClick={() => navigate('/statistics')}
                />
                <HeaderIcon
                    icon={settingsIcon}
                    title="Settings"
                    active={isActive('/settings')}
                    onClick={() => navigate('/settings')}
                />
            </Links>
            <Tray>
                <HeaderIcon
                    active={false}
                    icon={bellIcon}
                    title="Notifications"
                />
                <HeaderIcon
                    active={false}
                    icon={helpIcon}
                    title="Help"
                />
            </Tray>
        </Header>
    );
};
