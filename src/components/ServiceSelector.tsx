import { SERVICES, type ServiceName } from "../types";

interface Props {
  value: ServiceName;
  onChange: (service: ServiceName) => void;
}

export function ServiceSelector({ value, onChange }: Props) {
  return (
    <select
      className="service-selector"
      value={value}
      onChange={(e) => onChange(e.target.value as ServiceName)}
    >
      {SERVICES.map((s) => (
        <option key={s} value={s}>
          {s}
        </option>
      ))}
    </select>
  );
}
