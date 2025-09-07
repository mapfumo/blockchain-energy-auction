import React from "react";

interface LogoIconProps {
  size?: number;
  className?: string;
}

const LogoIcon: React.FC<LogoIconProps> = ({ size = 32, className = "" }) => {
  return (
    <img
      src="/favicon.ico?v=2"
      alt="Energy Trading Icon"
      width={size}
      height={size}
      className={className}
    />
  );
};

export default LogoIcon;
