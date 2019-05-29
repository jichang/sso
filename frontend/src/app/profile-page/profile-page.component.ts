import { Component, OnInit } from "@angular/core";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "profile-page",
  templateUrl: "./profile-page.component.html",
  styleUrls: ["./profile-page.component.css"]
})
export class ProfilePageComponent implements OnInit {
  constructor(private snackBar: MatSnackBar) {}

  ngOnInit() {}

  onProfileUpdateSuccess(evt) {
    this.snackBar.open("Profile update success", "Dismiss", {
      duration: 3000
    });
  }

  onProfileUpdateFailure(evt) {
    this.snackBar.open("Profile update failure", "Dismiss", {
      duration: 3000
    });
  }
}
