<?php

namespace Hungarofit\Evaluator;


use ReflectionClass;

class Enum
{
    const _DEFAULT = null;

    /** @var static[][] */
    private static $_instances = [];

    /**
     * @return ReflectionClass
     */
    protected static function _ref()
    {
        static $ref;
        if(!isset($ref)) {
            $ref = new ReflectionClass(static::class);
        }
        return $ref;
    }

    /**
     * @param null|string $name
     * @return array|mixed
     */
    protected static function _constants($name=null)
    {
        static $constants;
        if(!isset($constants)) {
            $constants = [];
        }
        if(!array_key_exists(static::class, $constants)) {
            $ref = new ReflectionClass(static::class);
            $constants[static::class] = $ref->getConstants();
        }
        if($name) {
            if(!self::hasName($name)) {
                throw new EnumException($name, EnumException::NAME);
            }
            return $constants[static::class][$name];
        }
        return $constants[static::class];
    }

    /**
     * @param string $name
     * @return bool
     */
    public static function hasName($name)
    {
        $c = self::_constants();
        $r = array_key_exists($name, $c);
        return $r;
    }

    /**
     * @param mixed $value
     * @return bool
     */
    public static function hasValue($value)
    {
        $c = self::_constants();
        $r = in_array($value, $c, true);
        return $r;
    }

    /**
     * @param mixed $value
     * @return string
     */
    public static function nameOfValue($value)
    {
        if(!self::hasValue($value)) {
            throw new EnumException($value, EnumException::VALUE);
        }
        return array_search($value, self::_constants());
    }

    /**
     * @param string $name
     * @return mixed
     */
    public static function valueOfName($name)
    {
        if(!self::hasName($name)) {
            throw new EnumException($name, EnumException::NAME);
        }
        return self::_constants($name);
    }

    /**
     * @param string $name
     * @return static
     */
    public static function fromName($name)
    {
        if(!self::hasName($name)) {
            throw new EnumException($name, EnumException::NAME);
        }
        if(!array_key_exists(static::class, self::$_instances)) {
            self::$_instances[static::class] = [];
        }
        if(!array_key_exists($name, self::$_instances[static::class])) {
            self::$_instances[static::class][$name] = new static($name);
        }
        return self::$_instances[static::class][$name];
    }

    /**
     * @param mixed $value
     * @return static
     */
    public static function fromValue($value)
    {
        $name = self::nameOfValue($value);
        return self::fromName($name);
    }

    /**
     * @param string $name
     * @param array $arguments
     * @return static
     */
    public static function __callStatic($name, $arguments=[])
    {
        return self::fromName($name);
    }

    /** @var string */
    protected $_name;

    /**
     * @param mixed $valueOrName
     */
    private function __construct($valueOrName = null)
    {
        if($valueOrName === null) {
            $valueOrName = static::_DEFAULT;
        }
        if($valueOrName === static::_DEFAULT || self::hasName($valueOrName)) {
            $this->_name = $valueOrName;
            return;
        }
        if(self::hasValue($valueOrName)) {
            $this->_name = self::nameOfValue($valueOrName);
            return;
        }
        throw new EnumException($valueOrName);
    }

    /**
     * @return string
     */
    public function getName()
    {
        return $this->_name;
    }

    /**
     * @return mixed
     */
    public function getValue()
    {
        return self::valueOfName($this->_name);
    }

    /**
     * @return string
     */
    public function __toString()
    {
        return (string)$this->getValue();
    }
}