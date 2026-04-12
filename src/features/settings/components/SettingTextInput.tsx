import Input from '@jetbrains/ring-ui-built/components/input/input';

interface SettingTextInputProps {
    label: string;
    value: string;
    help?: string;
    onChange: (value: string) => void;
}

export const SettingTextInput = ({label, value, help, onChange}: SettingTextInputProps) => {
    return (
        <Input
            label={label}
            value={value}
            help={help}
            onChange={event => onChange(event.currentTarget.value)}
        />
    );
};
