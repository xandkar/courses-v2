package cloud.ambar.creditCardProduct.exceptions;

import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.ResponseStatus;

@ResponseStatus(value = HttpStatus.BAD_REQUEST, reason = "Invalid Reward")
public class InvalidRewardException extends RuntimeException {
}
