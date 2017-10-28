<?php

namespace Hungarofit\Evaluator;


use InvalidArgumentException as Exception;
use Throwable;

class EnumException extends Exception
{
    const NAME = 1;
    const VALUE = 2;

    public function __construct($message = "", $code = 0, Throwable $previous = null)
    {
        switch ($code) {
            default:
                $message = 'Invalid enum: ' . $message;
                break;
            case self::NAME:
                $message = 'Invalid enum name: ' . $message;
                break;
            case self::VALUE:
                $message = 'Invalid enum value: ' . $message;
                break;
        }
        parent::__construct($message, $code, $previous);
    }
}