import { Component, OnInit } from "@angular/core";
import { TotpService, TotpStore } from "../totp.service";
import { FormBuilder, Validators, FormGroup } from "@angular/forms";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "two-fa-page",
  templateUrl: "./two-fa-page.component.html",
  styleUrls: ["./two-fa-page.component.css"]
})
export class TwoFaPageComponent implements OnInit {
  totpForm: FormGroup;
  totp?: TotpStore;

  constructor(
    private totpService: TotpService,
    private fb: FormBuilder,
    private snackBar: MatSnackBar
  ) {
    this.totpForm = fb.group({
      code: ["", [Validators.required]]
    });
  }

  ngOnInit() {
    this.totpService.qrcode.subscribe(totp => {
      this.totp = totp;
    });

    this.totpService.select();
  }

  setupTOTP({ value, valid }: { value: { code: string }; valid: boolean }) {
    this.totpService
      .update({
        code: parseInt(value.code, 10)
      })
      .subscribe(
        () => {
          this.snackBar.open("TOTP setup", "Dismiss", {
            duration: 3000
          });
        },
        err => {
          this.snackBar.open("TOTP failed", "Dismiss", {
            duration: 3000
          });
        }
      );
  }
}
