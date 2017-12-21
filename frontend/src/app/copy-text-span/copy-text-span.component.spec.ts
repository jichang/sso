import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { CopyTextSpanComponent } from './copy-text-span.component';

describe('CopyTextSpanComponent', () => {
  let component: CopyTextSpanComponent;
  let fixture: ComponentFixture<CopyTextSpanComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ CopyTextSpanComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CopyTextSpanComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should be created', () => {
    expect(component).toBeTruthy();
  });
});
