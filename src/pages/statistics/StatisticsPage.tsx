import Group from '@jetbrains/ring-ui-built/components/group/group';
import Island, {Content, Header} from '@jetbrains/ring-ui-built/components/island/island';
import Text from '@jetbrains/ring-ui-built/components/text/text';

export const StatisticsPage = () => {
    return (
        <Island className="w-full max-w-3xl">
            <Header border>Statistics</Header>
            <Content>
                <Group className="flex w-full flex-col gap-2">
                    <Text>statistics page is under construction.</Text>
                </Group>
            </Content>
        </Island>
    );
};
