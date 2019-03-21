import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { TotpFormComponent } from './totp-form.component';

describe('TotpFormComponent', () => {
  let component: TotpFormComponent;
  let fixture: ComponentFixture<TotpFormComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TotpFormComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(TotpFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
