<?php

namespace Hungarofit\Evaluator\Validate;


use Throwable;

class Exception extends \Exception
{
    protected $_errors = [];

    public function __construct($errors = [], $code = 0, Throwable $previous = null)
    {
        $this->_errors = $errors;
        $message = '';
        foreach($errors as $k=>$v) {
            $message.= $k . ' ' . lcfirst($v) . PHP_EOL;
        }
        parent::__construct($message, $code, $previous);
    }

    public function getErrors()
    {
        return $this->_errors;
    }
}